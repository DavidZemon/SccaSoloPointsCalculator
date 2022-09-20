import { Component, PropsWithoutRef } from 'react';
import { Accordion, Button, Card, Col, Row } from 'react-bootstrap';
import { RamDownload } from './DownloadButton';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDownload } from '@fortawesome/free-solid-svg-icons';
import { ChampionshipType } from 'scca_solo_points_engine/scca_solo_points_engine';
import { CsvTable } from './CsvTable';

interface ChampionshipResultsProps extends PropsWithoutRef<any> {
  results?: Partial<Record<keyof typeof ChampionshipType, string>>;
}

interface ChampionshipResultsState {
  downloadName?: string;
  downloadData?: BlobPart;
}

export class ChampionshipResults extends Component<
  ChampionshipResultsProps,
  ChampionshipResultsState
> {
  constructor(props: Readonly<ChampionshipResultsProps>) {
    super(props);
    this.state = {};
  }

  render() {
    if (
      this.props.results &&
      Object.values(this.props.results).some((v) => !!v)
    ) {
      return (
        <>
          <Row className={'top-buffer'}>
            <Col>
              <h2>Championship Standings</h2>

              <Accordion>
                {Object.entries(this.props.results)
                  // Class results need to be displayed separately
                  // Get some nice type hints going
                  .map(
                    ([key, csvContent]) =>
                      [key, csvContent] as [
                        keyof typeof ChampionshipType,
                        string,
                      ],
                  )
                  .map(([championshipType, csvContent]) => {
                    // eslint-disable-next-line @typescript-eslint/no-unused-vars
                    const [_region, title, _blank, ...data] =
                      csvContent.split('\n');
                    return (
                      <Card key={championshipType}>
                        <Card.Header key={championshipType}>
                          <Accordion.Toggle
                            eventKey={championshipType}
                            as={Button}
                            variant={'link'}
                          >
                            {title}
                          </Accordion.Toggle>
                          <Button
                            variant={'secondary'}
                            onClick={() => {
                              const titleWords = title.split(' ');
                              this.setState({
                                downloadData: csvContent,
                                downloadName: `${titleWords[0]}_StL_${titleWords[1]}_Championship.csv`,
                              });
                            }}
                          >
                            <FontAwesomeIcon
                              className={'clickable'}
                              icon={faDownload}
                            />
                          </Button>
                        </Card.Header>

                        <Accordion.Collapse eventKey={championshipType}>
                          <Card.Body>
                            <CsvTable
                              csv={data.join('\n')}
                              keyBuilder={(row, index) =>
                                `Championship ${championshipType} - ${row[1]} - ${index}`
                              }
                            />
                          </Card.Body>
                        </Accordion.Collapse>
                      </Card>
                    );
                  })}
              </Accordion>
            </Col>
          </Row>

          <RamDownload
            filename={this.state.downloadName}
            content={this.state.downloadData}
            contentType={'application/vnd.ms-excel'}
            downloadComplete={() =>
              this.setState({
                downloadData: undefined,
                downloadName: undefined,
              })
            }
          />
        </>
      );
    } else {
      return null;
    }
  }
}
